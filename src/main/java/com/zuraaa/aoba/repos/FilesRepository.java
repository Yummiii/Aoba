package com.zuraaa.aoba.repos;

import com.zuraaa.aoba.models.File;
import org.springframework.data.repository.CrudRepository;

public interface FilesRepository extends CrudRepository<File, String> {
}
