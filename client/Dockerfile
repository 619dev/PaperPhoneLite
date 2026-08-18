# ── Stage 1: Build (always on native arch, output is static files) ─────
FROM --platform=$BUILDPLATFORM node:20-alpine AS builder

WORKDIR /app
COPY package.json ./
RUN npm install
COPY . .
RUN npm run build

# ── Stage 2: Serve (multi-arch nginx) ──────────────────────────────────
FROM node:20-alpine

RUN npm install --global serve@14
COPY --from=builder /app/dist /app

EXPOSE 8080

CMD ["serve", "--single", "--listen", "8080", "/app"]
