package com.zuraaa.aoba.repos;

import com.zuraaa.aoba.models.User;
import org.springframework.data.repository.CrudRepository;

public interface UsersRepository extends CrudRepository<User, String> {
    User getByLastToken(String lastToken);
    User getByUsername(String username);
}
