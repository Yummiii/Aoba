package com.zuraaa.aoba.models;

import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;

import javax.persistence.*;

@Entity
@Table(name = "Files")
@AllArgsConstructor
@NoArgsConstructor
public class File {
    @Getter @Setter @Id
    private String id;
    @Getter @Setter
    private String fileName;
    @Getter @Setter
    private String mimeType;
    @Getter @Setter
    private boolean pub;
    @Getter @Setter
    private boolean pubListing;
    @Getter @Setter @ManyToOne @JoinColumn(name = "user_id", nullable = false)
    private User user;

}
