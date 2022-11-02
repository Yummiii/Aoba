package com.zuraaa.aoba.controllers;

import com.zuraaa.aoba.Configs;
import com.zuraaa.aoba.auth.JwtToken;
import com.zuraaa.aoba.models.FileData;
import com.zuraaa.aoba.models.FileMetadata;
import com.zuraaa.aoba.models.Folder;
import com.zuraaa.aoba.models.User;
import com.zuraaa.aoba.models.dto.FileUploadDto;
import com.zuraaa.aoba.repos.FilesDataRepository;
import com.zuraaa.aoba.repos.FilesMetadataRepository;
import com.zuraaa.aoba.repos.FoldersRepository;
import com.zuraaa.aoba.repos.UsersRepository;
import cool.graph.cuid.Cuid;
import lombok.AllArgsConstructor;
import org.jetbrains.annotations.NotNull;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.PageRequest;
import org.springframework.http.HttpHeaders;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.security.core.Authentication;
import org.springframework.security.core.context.SecurityContextHolder;
import org.springframework.web.bind.annotation.*;
import org.springframework.web.multipart.MultipartFile;

import javax.validation.Valid;
import java.net.URI;
import java.util.HashMap;
import java.util.List;
import java.util.Map;


@RestController
@RequestMapping("/files")
@AllArgsConstructor
public class FilesController {
    private UsersRepository usersRepo;
    private FilesMetadataRepository filesMetadataRepositoryRepo;
    private FoldersRepository foldersRepo;
    private FilesDataRepository filesDataRepo;
    private Configs configs;

    @PostMapping("/upload")
    public ResponseEntity<FileMetadata> addFile(@RequestBody @Valid FileUploadDto fileUp) throws Exception {
        Authentication auth = SecurityContextHolder.getContext().getAuthentication();
        JwtToken token = (JwtToken) auth.getPrincipal();
        User user = usersRepo.findById(token.getId()).orElseThrow();

        Folder folder;
        if (fileUp.getFolderId() == null || fileUp.getFolderId().isEmpty()) {
            folder = foldersRepo.getByNameAndUser("root", user);
        } else {
            folder = foldersRepo.findById(fileUp.getFolderId()).orElse(null);
            if (folder == null || !folder.getUser().getId().equals(user.getId())) {
                return new ResponseEntity<>(HttpStatus.UNAUTHORIZED);
            }
        }

        FileData data = filesDataRepo.save(new FileData(0, fileUp.getMimeType(), fileUp.getContent()));
        FileMetadata meta = new FileMetadata(Cuid.createCuid(), fileUp.getFileName(), fileUp.isPub(), fileUp.isPubList(), System.currentTimeMillis(), folder, user, data);
        meta = filesMetadataRepositoryRepo.save(meta);
        return ResponseEntity.created(URI.create("/files/" + meta.getId())).body(meta);
    }

    @GetMapping("/{id}/data")
    public ResponseEntity<Map<String, Object>> getFile(@PathVariable String id) throws Exception {
        User user;
        try {
            Authentication auth = SecurityContextHolder.getContext().getAuthentication();
            JwtToken token = (JwtToken) auth.getPrincipal();
            user = usersRepo.findById(token.getId()).orElseThrow();
        } catch (Exception e) {
            user = null;
        }

        FileMetadata meta = filesMetadataRepositoryRepo.findById(id).orElse(null);
        if (meta != null) {
            if (meta.isPub() || (user != null && user.getId().equals(meta.getUser().getId()))) {
                HttpHeaders headers = new HttpHeaders();
                headers.add("Content-Disposition", "filename=" + meta.getFileName());
                headers.add("Cache-Control", "max-age=2630000, no-transform");

                Map<String, Object> resp = new HashMap();
                resp.put("contentType", meta.getFileData().getMimeType());
                resp.put("content", meta.getFileData().getContent());
                return new ResponseEntity<>(resp, headers, HttpStatus.OK);
            } else {
                return new ResponseEntity<>(HttpStatus.UNAUTHORIZED);
            }
        } else {
            return new ResponseEntity<>(HttpStatus.BAD_REQUEST);
        }
    }

    @DeleteMapping("/delete/{id}")
    public ResponseEntity<String> deleteFile(@PathVariable String id) throws Exception {
        Authentication auth = SecurityContextHolder.getContext().getAuthentication();
        JwtToken token = (JwtToken) auth.getPrincipal();
        User user = usersRepo.findById(token.getId()).orElseThrow();
        FileMetadata meta = filesMetadataRepositoryRepo.findById(id).orElse(null);

        if (meta != null) {
            if (meta.getUser().getId().equals(user.getId())) {
                filesMetadataRepositoryRepo.delete(meta);
                filesDataRepo.delete(meta.getFileData());
                return ResponseEntity.ok().build();
            } else {
                return new ResponseEntity<>(HttpStatus.UNAUTHORIZED);
            }
        } else {
            return new ResponseEntity<>(HttpStatus.BAD_REQUEST);
        }
    }

    @GetMapping("/public")
    public ResponseEntity<Map<String, Object>> getPublic(@RequestParam(required = false, defaultValue = "0") int page, @RequestParam(required = false, defaultValue = "15") int size) {
        PageRequest paginator = PageRequest.of(Math.max(page - 1, 0), Math.max(size, 1));
        Page<FileMetadata> files = filesMetadataRepositoryRepo.findAllByPubListingAndPubOrderByCreatedAtDesc(true, true, paginator);
        Map<String, Object> resp = new HashMap<>();
        resp.put("totalElements", files.getTotalElements());
        resp.put("totalPages", files.getTotalPages());
        resp.put("files", files.getContent());
        return ResponseEntity.ok(resp);
    }
}
