package com.zuraaa.aoba.models;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.Setter;
import org.hibernate.validator.constraints.Length;

import javax.persistence.Entity;
import javax.persistence.Id;
import javax.persistence.Index;
import javax.persistence.Table;
import javax.validation.constraints.NotBlank;

@Entity
@Table(name = "Users", indexes = {@Index(columnList = "username", unique = true), @Index(columnList = "lastToken")})
@AllArgsConstructor
public class User {
    @Getter
    @Setter
    @Id
    private String id;
    @Getter
    @Setter
    @NotBlank(message = "Username is required")
    @Length(min = 3, max = 255)
    private String username;
    @Getter
    @Setter
    @NotBlank(message = "Password is required")
    @Length(min = 8)
    @JsonProperty(access = JsonProperty.Access.WRITE_ONLY)
    private String password;
    @Getter
    @Setter
    @JsonProperty(access = JsonProperty.Access.WRITE_ONLY)
    private String lastToken;

    public User() {

    }
}
