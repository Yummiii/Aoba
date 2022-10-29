package com.zuraaa.aoba.controllers;

import com.auth0.jwt.JWT;
import com.auth0.jwt.JWTCreator;
import com.auth0.jwt.algorithms.Algorithm;
import com.zuraaa.aoba.Configs;
import com.zuraaa.aoba.auth.JwtToken;
import com.zuraaa.aoba.models.FileData;
import com.zuraaa.aoba.models.FileMetadata;
import com.zuraaa.aoba.models.Folder;
import com.zuraaa.aoba.models.User;
import com.zuraaa.aoba.repos.FilesDataRepository;
import com.zuraaa.aoba.repos.FilesMetadataRepository;
import com.zuraaa.aoba.repos.FoldersRepository;
import com.zuraaa.aoba.repos.UsersRepository;
import cool.graph.cuid.Cuid;
import lombok.AllArgsConstructor;
import lombok.SneakyThrows;
import org.jetbrains.annotations.NotNull;
import org.springframework.http.HttpHeaders;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.security.core.Authentication;
import org.springframework.security.core.context.SecurityContextHolder;
import org.springframework.security.crypto.password.PasswordEncoder;
import org.springframework.web.bind.annotation.*;
import org.springframework.web.multipart.MultipartFile;

import javax.validation.Valid;
import java.net.URI;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

@RestController
@RequestMapping("/users")
@AllArgsConstructor
public class UsersController {
    private PasswordEncoder passwordEncoder;
    private UsersRepository usersRepo;
    private FoldersRepository foldersRepo;
    private FilesDataRepository filesDataRepo;
    private FilesMetadataRepository filesMetadataRepo;
    private Configs configs;

    @PostMapping("/create")
    public ResponseEntity<User> createUser(@Valid @RequestBody User create_user) throws Exception {
        if (usersRepo.getByUsername(create_user.getUsername()) == null) {
            User user = usersRepo.save(new User(Cuid.createCuid(), create_user.getUsername(), passwordEncoder.encode(create_user.getPassword()), null, null, null, null));
            foldersRepo.save(new Folder(Cuid.createCuid(), null, null, "root", null, user));
            return ResponseEntity.created(URI.create("/users/" + user.getId())).body(user);
        } else {
            return new ResponseEntity<>(HttpStatus.CONFLICT);
        }
    }

    @PostMapping("/authenticate")
    public ResponseEntity<Map<String, Object>> authenticate(@RequestBody User auth_user) {
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

    @GetMapping("/{id}/list")
    public ResponseEntity<Map<String, Object>> getFiles(@PathVariable String id, @RequestParam(name = "folder", required = false) String folderId) {
        Authentication auth = SecurityContextHolder.getContext().getAuthentication();
        User user = null;
        boolean login = false;
        if (id.equals("@me")) {
            if (!auth.getPrincipal().toString().equals("anonymousUser")) {
                JwtToken token = (JwtToken) auth.getPrincipal();
                user = usersRepo.findById(token.getId()).orElse(null);
                login = true;
            }
        } else {
            user = usersRepo.findById(id).orElse(null);
        }

        if (user != null) {
            if (!auth.getPrincipal().toString().equals("anonymousUser")) {
                JwtToken token = (JwtToken) auth.getPrincipal();
                if (user.getId().equals(token.getId())) {
                    login = true;
                }
            }

            Folder folder = null;
            if (folderId == null || folderId.isEmpty()) {
                folder = foldersRepo.getByNameAndUser("root", user);
            } else {
                folder = foldersRepo.findByIdAndUser(folderId, user).orElse(null);
            }

            if (folder != null) {
                Map<String, Object> resp = new HashMap<>();
                List<FileMetadata> files = null;
                if (login) {
                    files = filesMetadataRepo.findByFolderAndUserOrderByCreatedAtDesc(folder, user);
                } else {
                    files = filesMetadataRepo.findByFolderAndUserAndPubListingOrderByCreatedAtDesc(folder, user, true);
                }
                resp.put("files", files);
                resp.put("folders", foldersRepo.findByParentAndUser(folder, user));
                return ResponseEntity.ok().body(resp);
            } else {
                return new ResponseEntity<>(HttpStatus.BAD_REQUEST);
            }
        } else {
            return new ResponseEntity<>(HttpStatus.BAD_REQUEST);
        }
    }

    @PostMapping("/avatar")
    @SneakyThrows
    public ResponseEntity<String> setAvatar(@RequestPart @NotNull MultipartFile file, @RequestPart @NotNull String mimeType) {
        Authentication auth = SecurityContextHolder.getContext().getAuthentication();
        JwtToken token = (JwtToken) auth.getPrincipal();
        User user = usersRepo.findById(token.getId()).orElse(null);

        if (user != null) {
            FileData avatar = filesDataRepo.save(new FileData(0, mimeType, file.getBytes()));
            user.setAvatar(avatar);
            usersRepo.save(user);
            return ResponseEntity.ok().build();
        } else {
            return new ResponseEntity<>(HttpStatus.UNAUTHORIZED);
        }
    }

    @GetMapping("/{id}/avatar")
    public ResponseEntity<byte[]> getAvatar(@PathVariable String id) {
        User user = usersRepo.findById(id).orElse(null);
        if (user != null && user.getAvatar() != null) {
            HttpHeaders headers = new HttpHeaders();
            headers.add("Content-Disposition", "filename=" + user.getId());
            headers.add("Content-Type", user.getAvatar().getMimeType());
            return new ResponseEntity<>(user.getAvatar().getContent(), headers, HttpStatus.OK);
        } else {
            return ResponseEntity.badRequest().build();
        }
    }
}
