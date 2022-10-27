package com.zuraaa.aoba.repos;

import com.zuraaa.aoba.models.Folder;
import com.zuraaa.aoba.models.User;
import org.springframework.data.repository.CrudRepository;

import java.util.List;

public interface FoldersRepository extends CrudRepository<Folder, String> {
    Folder getByNameAndUser(String name, User user);
    List<Folder> getByUser(User user);
}
