# Colgado

***Idioma***

- 🇪🇸 Español
- [🇺🇸 English](./README.md)

Un juego para jugar en twitch basado en el juego del ahorcado en el que el chat tiene que adivinar una palabra que el streamer ha introducido previamente.

## Instrucciones
### Compilar el codigo
Si ya tienes el ejecutable puedes saltarte este paso y ve directamente a la sección de crear una aplicación.
1. Clonar el repositorio (Si no tienes git instalado, pulsa el botón de "Code" y "Download ZIP")
2. Instalar Rust, para ello sigue la guía de instalación oficial https://www.rust-lang.org/es/tools/install .
3. Compilar el programa, para ello abre la consola de comandos y ejecuta el siguiente comando:
```bash
cargo build --release
```
4. Ir al directorio `target/release` el ejecutable se llama `colgado` si estas en linux o `colgado.exe` en windows.
### Crear una aplicación de twitch

Para ello simplemente hay que ir a https://dev.twitch.tv/console/apps/create y rellenarlo de la siguiente manera y pulsamos en Crear:
![app-reg](./assets/reg_app.png)

Una vez hecho esto estaremos en la pagina https://dev.twitch.tv/console/apps en la que aparecen las aplicaciones creadas.

Pulsamos en el botón administrar de nuestro bot, en esta pantalla veremos el id de cliente y las urls de redireccionamiento (en nuestro caso solo una).

En el mismo directorio desde el que ejecutamos el juego creamos el siguiente fichero nombrandolo como "env.toml" teniendo en cuenta que "toml" es la extensión:

```toml
client-id = "h8h9gg6gu59m0187lvgy01x6teinig"
redirect-urls = [
    "http://localhost:3000/esto-es-un-texto-muy-largo-para-que-no-se-vea-el-access-token-que-en-el-caso-de-que-estes-enseñando-el-navegador-en-directo-seria-un-gran-problema-por-favor-ten-cuidado",
    "http://localhost:1234/esto-es-un-texto-muy-largo-para-que-no-se-vea-el-access-token-que-en-el-caso-de-que-estes-enseñando-el-navegador-en-directo-seria-un-gran-problema-por-favor-ten-cuidado",
    "http://localhost:8000/esto-es-un-texto-muy-largo-para-que-no-se-vea-el-access-token-que-en-el-caso-de-que-estes-enseñando-el-navegador-en-directo-seria-un-gran-problema-por-favor-ten-cuidado",
]
command = "!colgado"
```

En él:

- Introducimos en el client-id el id de nuestro bot, es público, por lo que no hay problema en compartirlo

- Se ha añadido texto porque cuando se inicie sesión aparecera en la url un token que  **no se debe compartir con nadie**

- `command` es la palabra que se utilizará para distinguir los mensajes relacionados con el juego de los demás

Modificamos la url de nuestro bot y añadimos las otras dos.

![edit-bot](./assets/editando_bot.png)

## Que hacer si se me filtra el token

Ir a esta dirección https://www.twitch.tv/settings/connections y en la sección Otras Conexiones pulsar en el botón de "Desconectar" del bot.
