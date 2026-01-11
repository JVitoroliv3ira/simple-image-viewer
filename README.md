# simple-image-viewer (siv)

Um visualizador de imagens **bem simples**, feito em **Rust**.

> **Projeto experimental / de aprendizado**  
> Este projeto é apenas para estudo.  
> Não é um visualizador completo e não pretende ser um.

---

## O que ele faz (v0)

- Abre uma janela
- Carrega uma imagem a partir do caminho passado por argumento
- Renderiza a imagem na tela
- Fecha ao clicar no botão de fechar da janela

---

## Uso

### Via Cargo
```bash
cargo run -- ./caminho/para/imagem.png
````

### Via Makefile

```bash
make build-run ARGS="./caminho/para/imagem.png"
```

Ou em release:

```bash
make run-release ARGS="./caminho/para/imagem.png"
```

---

## Exemplo

```bash
make build-run ARGS="./examples/image.png"
```

---

## Licença

Este projeto é licenciado sob a **GNU General Public License v3.0 (GPL-3.0)**.

Consulte o arquivo `LICENSE` para mais detalhes.
