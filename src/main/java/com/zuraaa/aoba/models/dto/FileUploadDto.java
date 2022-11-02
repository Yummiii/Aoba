package com.zuraaa.aoba.models.dto;

import lombok.Getter;
import lombok.Setter;

import javax.validation.constraints.NotNull;

public class FileUploadDto {
    @Getter @Setter @NotNull
    private String mimeType;
    @Getter @Setter
    private String folderId;
    @Getter @Setter
    private String fileName;
    @Getter @Setter
    private boolean pub;
    @Getter @Setter
    private boolean pubList;
    @Getter @Setter @NotNull
    private String content;
}
