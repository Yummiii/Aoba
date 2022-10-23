package com.zuraaa.aoba.auth;

import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.Setter;

@AllArgsConstructor
public class JwtToken {
    @Getter
    @Setter
    private String id;

//    public String createToken(String secret) {
//        Algorithm algorithm = Algorithm.HMAC256(secret);
//
//        JWTCreator.Builder builder = JWT.create();
//        builder.withClaim("id", id);
//
//        return builder.sign(algorithm);
//    }
}
