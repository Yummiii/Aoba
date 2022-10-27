package com.zuraaa.aoba;

import lombok.Getter;
import lombok.Setter;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.boot.context.properties.ConfigurationProperties;
import org.springframework.boot.context.properties.ConfigurationPropertiesScan;

@ConfigurationPropertiesScan
@ConfigurationProperties(prefix = "aoba")
public class Configs {
    @Getter
    @Setter
    @Value("${jwt.secret:ChangeMe}")
    private String jwtSecret;
}
