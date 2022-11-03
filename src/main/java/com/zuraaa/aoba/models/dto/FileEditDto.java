package com.zuraaa.aoba.models.dto;

import lombok.Getter;
import lombok.Setter;
import org.springframework.beans.factory.annotation.Value;

import java.util.Optional;

public class FileEditDto {
    @Getter @Setter
    private Boolean pub;
    @Getter @Setter @Value(value = "null")
    private Boolean pubList;
    @Getter @Setter
    private String content;
}
