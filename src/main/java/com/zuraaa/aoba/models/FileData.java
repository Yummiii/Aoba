package com.zuraaa.aoba.models;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

import javax.persistence.*;

@Entity
@Table(name = "FilesData")
@AllArgsConstructor
@NoArgsConstructor
public class FileData {
    @Getter @Setter @Id @GeneratedValue(strategy = GenerationType.SEQUENCE)
    private long id;
    @Getter @Setter
    private String mimeType;
    @Getter @Setter @JsonProperty(access = JsonProperty.Access.WRITE_ONLY)
    private byte[] content;
//    @Getter @Setter @OneToOne
//    private FileMetadata metadata;
}
