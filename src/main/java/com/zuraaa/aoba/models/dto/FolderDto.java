package com.zuraaa.aoba.models.dto;

import lombok.Getter;
import lombok.Setter;

import javax.validation.constraints.NotNull;

public class FolderDto {
    @Getter @Setter @NotNull
    private String name;
    @Getter @Setter
    private String parent;
}
