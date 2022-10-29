package com.zuraaa.aoba.repos;

import com.zuraaa.aoba.models.FileMetadata;
import com.zuraaa.aoba.models.Folder;
import com.zuraaa.aoba.models.User;
import org.springframework.data.repository.CrudRepository;

import java.util.List;

public interface FilesMetadataRepository extends CrudRepository<FileMetadata, String> {
    List<FileMetadata> findByFolderAndUserAndPubListingOrderByCreatedAtDesc(Folder folder, User user, boolean pubListing);
    List<FileMetadata> findByFolderAndUserOrderByCreatedAtDesc(Folder folder, User user);
}
