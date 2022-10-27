package com.zuraaa.aoba.models;

import com.fasterxml.jackson.annotation.JsonIgnore;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

import javax.persistence.*;
import java.util.List;

@Entity
@Table(name = "Folders")
@AllArgsConstructor
@NoArgsConstructor
public class Folder {
    @Getter @Setter @Id
    private String id;
    @Getter @Setter @ManyToOne
    private Folder parent;
    @Getter @Setter @OneToMany(mappedBy = "parent")
    private List<Folder> children;
    @Getter @Setter
    private String name;
    @Getter @Setter @OneToMany(mappedBy = "folder")
    private List<FileMetadata> files;
    @Getter @Setter @ManyToOne @JoinColumn(name = "user_id", nullable = false) @JsonIgnore
    private User user;
}
