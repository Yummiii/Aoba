package com.zuraaa.aoba.models;

import lombok.Getter;
import lombok.Setter;

import javax.persistence.*;

@Entity
@Table(name = "Files")
public class File {
    @Getter @Setter @Id
    private String id;
    @Getter @Setter
    private boolean pub;
    @Getter @Setter
    private boolean pubListing;

    @Getter @Setter @ManyToOne @JoinColumn(name = "user_id", nullable = false)
    private User user;

}
