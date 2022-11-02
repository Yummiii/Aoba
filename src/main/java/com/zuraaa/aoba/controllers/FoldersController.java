package com.zuraaa.aoba.controllers;

import com.zuraaa.aoba.auth.JwtToken;
import com.zuraaa.aoba.models.Folder;
import com.zuraaa.aoba.models.dto.FolderDto;
import com.zuraaa.aoba.models.User;
import com.zuraaa.aoba.repos.FoldersRepository;
import com.zuraaa.aoba.repos.UsersRepository;
import cool.graph.cuid.Cuid;
import lombok.AllArgsConstructor;
import org.springframework.http.ResponseEntity;
import org.springframework.security.core.Authentication;
import org.springframework.security.core.context.SecurityContextHolder;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import javax.validation.Valid;
import java.net.URI;

@RestController
@RequestMapping("/folders")
@AllArgsConstructor
public class FoldersController {
    private UsersRepository usersRepo;
    private FoldersRepository foldersRepo;

    @PostMapping("/create")
    public ResponseEntity<Folder> createFolder(@Valid @RequestBody FolderDto folder) {
        Authentication auth = SecurityContextHolder.getContext().getAuthentication();
        JwtToken token = (JwtToken) auth.getPrincipal();
        User user = usersRepo.findById(token.getId()).orElseThrow();

        Folder parent = null;
        if (folder.getParent() != null) {
            parent = foldersRepo.findById(folder.getParent()).orElse(null);
        }

        if (parent == null) {
            parent = foldersRepo.getByNameAndUser("root", user);
        }

        Folder saved = foldersRepo.save(new Folder(Cuid.createCuid(), parent, null, folder.getName(), null, user));
        return ResponseEntity.created(URI.create("/users/@me/list?folder=" + saved.getId())).body(saved);
    }
}
