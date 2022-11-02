package com.zuraaa.aoba.repos;

import com.zuraaa.aoba.models.FileMetadata;
import com.zuraaa.aoba.models.Folder;
import com.zuraaa.aoba.models.User;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.CrudRepository;

public interface FilesMetadataRepository extends CrudRepository<FileMetadata, String> {
    Page<FileMetadata> findByFolderAndUserAndPubListingOrderByCreatedAtDesc(Folder folder, User user, boolean pubListing, Pageable pageable);

    Page<FileMetadata> findByFolderAndUserOrderByCreatedAtDesc(Folder folder, User user, Pageable pageable);

    Page<FileMetadata> findAllByPubListingAndPubOrderByCreatedAtDesc(boolean pubListing, boolean pub, Pageable pageable);
    @Query(value = "select * from files_metadata where pub = true and pub_listing = true order by random() limit 1", nativeQuery = true)
    FileMetadata getRandomPublic();
}
