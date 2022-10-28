package com.zuraaa.aoba.repos;

import com.zuraaa.aoba.models.Folder;
import com.zuraaa.aoba.models.User;
import org.springframework.data.repository.CrudRepository;

import java.util.List;
import java.util.Optional;

public interface FoldersRepository extends CrudRepository<Folder, String> {
    Folder getByNameAndUser(String name, User user);
    List<Folder> findByParentAndUser(Folder parent, User user);
    Optional<Folder> findByIdAndUser(String id, User user);
}
