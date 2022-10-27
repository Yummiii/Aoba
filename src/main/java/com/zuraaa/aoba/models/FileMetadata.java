package com.zuraaa.aoba.models;

import com.fasterxml.jackson.annotation.JsonIgnore;
import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

import javax.persistence.*;

@Entity
@Table(name = "FilesMetadata")
@AllArgsConstructor
@NoArgsConstructor
public class FileMetadata {
    @Getter
    @Setter
    @Id
    private String id;
    @Getter
    @Setter
    private String fileName;
    @Getter
    @Setter
    private boolean pub;
    @Getter
    @Setter
    private boolean pubListing;
    @Getter
    @Setter
    @ManyToOne
    @JoinColumn(name = "folder_id", nullable = false)
    @JsonIgnore
    private Folder folder;
    @Getter
    @Setter
    @ManyToOne
    @JoinColumn(name = "user_id", nullable = false)
    @JsonIgnore
    private User user;
    @Getter
    @Setter
    @OneToOne(cascade = CascadeType.ALL)
    @JoinColumn(name = "file_data_id", referencedColumnName = "id")
    @JsonIgnore
    private FileData fileData;
}
