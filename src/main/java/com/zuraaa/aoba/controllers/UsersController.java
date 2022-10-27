package com.zuraaa.aoba.controllers;

import com.auth0.jwt.JWT;
import com.auth0.jwt.JWTCreator;
import com.auth0.jwt.algorithms.Algorithm;
import com.zuraaa.aoba.Configs;
import com.zuraaa.aoba.auth.JwtToken;
import com.zuraaa.aoba.models.FileMetadata;
import com.zuraaa.aoba.models.Folder;
import com.zuraaa.aoba.models.User;
import com.zuraaa.aoba.repos.FoldersRepository;
import com.zuraaa.aoba.repos.UsersRepository;
import cool.graph.cuid.Cuid;
import lombok.AllArgsConstructor;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.security.core.Authentication;
import org.springframework.security.core.context.SecurityContextHolder;
import org.springframework.security.crypto.password.PasswordEncoder;
import org.springframework.web.bind.annotation.*;

import javax.validation.Valid;
import java.net.URI;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;
import java.util.Map;

@RestController
@RequestMapping("/users")
@AllArgsConstructor
public class UsersController {
    private PasswordEncoder passwordEncoder;
    private UsersRepository usersRepo;
    private FoldersRepository foldersRepo;
    private Configs configs;

    @PostMapping("/create")
    public ResponseEntity<User> createUser(@Valid @RequestBody User create_user) throws Exception {
        if (usersRepo.getByUsername(create_user.getUsername()) == null) {
            User user = usersRepo.save(new User(Cuid.createCuid(), create_user.getUsername(), passwordEncoder.encode(create_user.getPassword()), null, null, null));
            foldersRepo.save(new Folder(Cuid.createCuid(), null, null, "root", null, user));
            return ResponseEntity.created(URI.create("/users/" + user.getId())).body(user);
        } else {
            return ResponseEntity.badRequest().build();
        }
    }

    @PostMapping("/authenticate")
    public ResponseEntity<Map<String, Object>> authenticate(@Valid @RequestBody User auth_user) {
        User user = usersRepo.getByUsername(auth_user.getUsername());
        if (user != null) {
            if (passwordEncoder.matches(auth_user.getPassword(), user.getPassword())) {
                Algorithm algorithm = Algorithm.HMAC256(configs.getJwtSecret());
                JWTCreator.Builder builder = JWT.create();
                builder.withIssuer("Aoba");
                builder.withClaim("id", user.getId());
                String token = builder.sign(algorithm);

                user.setLastToken(token);
                usersRepo.save(user);
                return ResponseEntity.ok().body(Map.of("token", token));
            } else {
                return new ResponseEntity<>(HttpStatus.UNAUTHORIZED);
            }
        } else {
            return new ResponseEntity<>(HttpStatus.UNAUTHORIZED);
        }
    }

    @GetMapping("/{id}/files")
    public ResponseEntity<List<Folder>> getFiles(@PathVariable String id) {
        Authentication auth = SecurityContextHolder.getContext().getAuthentication();
        JwtToken token = (JwtToken) auth.getPrincipal();
        User user = usersRepo.findById(id.equals("@me") ? token.getId() : id).orElse(null);

        if (user != null) {
            return ResponseEntity.ok(foldersRepo.getByUser(user));
        } else {
            return new ResponseEntity<>(HttpStatus.BAD_REQUEST);
        }
    }
}
