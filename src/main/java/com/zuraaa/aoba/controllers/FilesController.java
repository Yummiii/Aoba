package com.zuraaa.aoba.controllers;

import com.zuraaa.aoba.Configs;
import com.zuraaa.aoba.auth.JwtToken;
import com.zuraaa.aoba.models.File;
import com.zuraaa.aoba.models.User;
import com.zuraaa.aoba.repos.FilesRepository;
import com.zuraaa.aoba.repos.UsersRepository;
import cool.graph.cuid.Cuid;
import lombok.AllArgsConstructor;
import org.jetbrains.annotations.NotNull;
import org.springframework.http.HttpHeaders;
import org.springframework.http.HttpStatus;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.security.core.Authentication;
import org.springframework.security.core.context.SecurityContextHolder;
import org.springframework.web.bind.annotation.*;
import org.springframework.web.multipart.MultipartFile;

import java.net.URI;
import java.nio.file.Files;
import java.nio.file.Paths;

@RestController
@RequestMapping("/files")
@AllArgsConstructor
public class FilesController {
    private UsersRepository usersRepo;
    private FilesRepository filesRepo;
    private Configs configs;

    @PostMapping("/add")
    public ResponseEntity<File> addFile(@RequestPart @NotNull MultipartFile file, @RequestPart String mimeType, @RequestPart String pub, @RequestPart String pubList) throws Exception {
        Authentication auth = SecurityContextHolder.getContext().getAuthentication();
        JwtToken token = (JwtToken) auth.getPrincipal();
        User user = usersRepo.findById(token.getId()).orElseThrow();
        File fileDb = filesRepo.save(new File(Cuid.createCuid(), file.getOriginalFilename(), mimeType, Boolean.parseBoolean(pub), Boolean.parseBoolean(pubList), user));
        file.transferTo(Paths.get(configs.getBasePath(), user.getId(), fileDb.getId()));
        return ResponseEntity.created(URI.create("/files/" + fileDb.getId())).body(fileDb);
    }

    @GetMapping("/data/{id}")
    public ResponseEntity<byte[]> getFile(@PathVariable String id) throws Exception {
        User user;
        try {
            Authentication auth = SecurityContextHolder.getContext().getAuthentication();
            JwtToken token = (JwtToken) auth.getPrincipal();
            user = usersRepo.findById(token.getId()).orElseThrow();
        } catch (Exception e) {
            user = null;
        }

        File file = filesRepo.findById(id).orElseThrow();
        if (file.isPub() || (user != null && user.getId().equals(file.getUser().getId()))) {
            HttpHeaders headers = new HttpHeaders();
            headers.setContentType(MediaType.parseMediaType(file.getMimeType()));
            return new ResponseEntity<>(Files.readAllBytes(Paths.get(configs.getBasePath(), file.getUser().getId(), file.getId())), headers, HttpStatus.OK);
        } else {
            return new ResponseEntity<>(HttpStatus.UNAUTHORIZED);
        }
    }
}
