package com.zuraaa.aoba.auth;

import com.zuraaa.aoba.Configs;
import com.zuraaa.aoba.repos.UsersRepository;
import lombok.AllArgsConstructor;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.security.config.annotation.web.builders.HttpSecurity;
import org.springframework.security.config.annotation.web.configuration.EnableWebSecurity;
import org.springframework.security.config.http.SessionCreationPolicy;
import org.springframework.security.crypto.bcrypt.BCryptPasswordEncoder;
import org.springframework.security.crypto.password.PasswordEncoder;
import org.springframework.security.web.SecurityFilterChain;
import org.springframework.security.web.authentication.UsernamePasswordAuthenticationFilter;
import org.springframework.web.cors.CorsConfiguration;
import org.springframework.web.cors.CorsConfigurationSource;
import org.springframework.web.cors.UrlBasedCorsConfigurationSource;

@Configuration
@AllArgsConstructor
@EnableWebSecurity
public class SecurityConfig {
    private Configs configs;
    private UsersRepository usersRepo;

    @Bean
    public SecurityFilterChain filterChain(HttpSecurity http) throws Exception {
        http.csrf().disable();
        http.cors().configurationSource(x -> {
            CorsConfiguration corsCfg = new CorsConfiguration();

            corsCfg.addAllowedOriginPattern("*");
            corsCfg.addExposedHeader("Location");
            corsCfg.addAllowedHeader("*");
            corsCfg.addAllowedMethod(CorsConfiguration.ALL);

            return corsCfg;
        });
        http.sessionManagement().sessionCreationPolicy(SessionCreationPolicy.STATELESS);

        http.authorizeRequests(auth -> {
            auth.antMatchers("/users/authenticate").permitAll();
            auth.antMatchers("/users/create").permitAll();
            auth.antMatchers("/users/*/list").permitAll();
            auth.antMatchers("/users/*/avatar").permitAll();
            auth.antMatchers("/files/*/data").permitAll();
            auth.antMatchers("/files/public").permitAll();
            auth.antMatchers("/files/public/*").permitAll();
            auth.anyRequest().authenticated();
        });

        http.addFilterBefore(new JwtTokenFilter(configs, usersRepo), UsernamePasswordAuthenticationFilter.class);
        return http.build();
    }

    @Bean
    public PasswordEncoder passwordEncoder() {
        return new BCryptPasswordEncoder();
    }
}
