package com.zuraaa.aoba.auth;

import com.auth0.jwt.JWT;
import com.auth0.jwt.JWTVerifier;
import com.auth0.jwt.algorithms.Algorithm;
import com.auth0.jwt.exceptions.JWTVerificationException;
import com.auth0.jwt.interfaces.DecodedJWT;
import com.zuraaa.aoba.Configs;
import com.zuraaa.aoba.models.User;
import com.zuraaa.aoba.repos.UsersRepository;
import lombok.AllArgsConstructor;
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken;
import org.springframework.security.core.context.SecurityContextHolder;
import org.springframework.stereotype.Component;
import org.springframework.web.filter.OncePerRequestFilter;

import javax.servlet.FilterChain;
import javax.servlet.ServletException;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;

@Component
@AllArgsConstructor
public class JwtTokenFilter extends OncePerRequestFilter {
    private Configs configs;
    private UsersRepository usersRepo;

    @Override
    protected void doFilterInternal(HttpServletRequest request, HttpServletResponse response, FilterChain chain) throws ServletException, IOException {
        String token = request.getHeader("Authorization");
        if (token != null) {
            token = token.substring(7).trim();
            Algorithm algorithm = Algorithm.HMAC256(configs.getJwtSecret());
            JWTVerifier verifier = JWT.require(algorithm).withIssuer("Aoba").build();

            try {
                DecodedJWT jwt = verifier.verify(token);
                String claim_id = jwt.getClaim("id").asString();
                User user = usersRepo.getByLastToken(token);

                if (user != null && user.getId().equals(claim_id)) {
                    UsernamePasswordAuthenticationToken auth = new UsernamePasswordAuthenticationToken(new JwtToken(claim_id), null, null);
                    SecurityContextHolder.getContext().setAuthentication(auth);
                }
            } catch (JWTVerificationException exception) {
                chain.doFilter(request, response);
                return;
            }
        }
        chain.doFilter(request, response);
    }
}
