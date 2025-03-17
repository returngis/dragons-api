# API de Dragones

¡Hola desarrollador 👋🏻! Este es un repositorio donde he creado diferentes Issues con la ayuda de GitHub Copilot y con todas las funcionalidades que puedes usar hoy con Issues y Projects.

También contiene una API RESTful para gestionar dragones, escrita en Rust y usando MongoDB como base de datos. Puedes usar esta API para crear, leer, actualizar y eliminar dragones. 🐉✨

## Características Principales ✨

- **Crear Dragones**: Agrega nuevos dragones a la base de datos con detalles como nombre, tipo y habilidades. 🐲
- **Leer Dragones**: Obtén información sobre todos los dragones o un dragón específico por ID. 📖
- **Actualizar Dragones**: Modifica registros existentes de dragones con nueva información. ✏️
- **Eliminar Dragones**: Elimina dragones de la base de datos. 🗑️

## Tecnologías Utilizadas 🛠️

- **Rust**: Un lenguaje de programación rápido y eficiente en el uso de memoria.
- **Rocket**: Un framework web para Rust que facilita la creación de aplicaciones web rápidas y seguras.
- **MongoDB**: Una base de datos NoSQL que almacena datos en documentos flexibles similares a JSON.

## Ejemplos de Endpoints 🛣️

- `POST /dragons`: Crea un nuevo dragón.
- `GET /dragons`: Obtén una lista de todos los dragones.
- `GET /dragons/:id`: Obtén detalles de un dragón específico.
- `PUT /dragons/:id`: Actualiza la información de un dragón.
- `DELETE /dragons/:id`: Elimina un dragón.

¡Esta API es perfecta para cualquier aplicación con temática de fantasía que necesite gestionar una colección de dragones! 🐉✨

## Cómo ejecutar 🚀

Tienes un par de opciones para ejecutar este proyecto:

1. **Ejecutar Localmente**:
    - Clona el repositorio: `git clone https://github.com/dragonstone-org/dragons-api.git`
    - Usa Dev Containers con Visual Studio Code.
    - Ejecuta el proyecto con `cargo run`.

2. **Ejecutar en GitHub Codespaces**:
     - Haz clic en el botón "Code" y selecciona "Open with Codespaces".
     - Espera a que el entorno se construya y el proyecto se compile.
     - Accede a la API en la URL proporcionada.
