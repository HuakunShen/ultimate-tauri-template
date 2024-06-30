# Ultimate Tauri Monorepo Template

Setting up a complex Tauri App can be difficult when many features have to be implemented with Rust. 

This template is a monorepo that contains a Tauri App with features I usually need in my project.

## Features

### Server

I sometimes need to implement apps that need to communicate with other apps in the same local network, or from localhost. This must be done with a server.

gRPC is a good choice for this as it's fast and have type safety.

Scanning the local network by going over all IP addresses is slow and CPU intensive. UDP Discovery through broadcast is implemented to find other servers under the same network.

- [x] HTTP Server
- [x] HTTPS Server
- [x] gRPC Server
- [x] UDP Discovery

### Frontend

I chose Nuxt for the frontend of the desktop app. 

I am personally familiar with React, Vue, and Svelte. In terms of syntax and features, I like Svelte the most. 

I would rate the Developer experience as: **Svelete > Vue > React**

However, ecosystem and community support are important, especially when complex UI components are needed, such as diagrams, UI components.

I would rate the Ecosystem as: **React > Vue > Svelte**.

The reason I chose a meta-framework like Nuxt is
1. It comes with a lot of extra features that I enjoy, such as file-based routing and static site generation.
2. Single Page Application like pure React or Vue can grow very large, and I will have to manually split code and lazy load components/APIs to reduce bundle size. Although files are served from local, a SPA still needs to load JS first before rendering, leading to larger delay as app size grows. 
   I always write Tauri apps with SvelteKit and Nuxt because they support SSG. Pages are pre-rendered and served as static HTML files, which is very fast.
   I don't consider NextJs because it's simply too hard to use, especially with Tauri. I had wasted too much time trying to make it work. Things like `"use client;"` and error boundary always cause problems, but I never need to worry about these with Nuxt and SvelteKit.

That's why Nuxt is my choice for Tauri app. It falls in middle in both Developer Experience and Ecosystem. Although in middle, it's already very good.

- [x] Nuxt Frontend
- [x] TailwindCSS

### Database

I need a database to store data. SQLite is a good choice because it's fast and easy to use. However ORM and encryption is a headache when using Rust. I will think about this later.

- [ ] SQLite
- [ ] Encrypted SQLite

