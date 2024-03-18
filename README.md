## Clonando o Repositório com Submódulos

Certifique-se de que o Git está instalado em seu sistema antes de prosseguir.

Para clonar este repositório e seus submódulos, execute o seguinte comando:

```shell
git clone --recurse-submodules https://github.com/guigo613/nmea_parse.git
```

Se você já clonou o repositório sem os submódulos, pode atualizá-los usando:

```shell
git submodule update --init --recursive
```
