Manual para Ejecutar el Proyecto Rust-Calculator:

A continuación, te presento un manual completo para ejecutar el proyecto de la calculadora escrita en el lenguaje de prgramación Rust. Asegúrate de seguir los pasos y tener todas las dependencias correctamente configuradas en tu proyecto:

Requisitos Previos:

    • Rust y Cargo: Asegúrate de tener Rust y Cargo instalados en tu sistema. Si no los tienes, puedes obtenerlos siguiendo las instrucciones en el sitio oficial de Rust: https://www.rust-lang.org/tools/install.
    • 
    • GTK Bibliotecas: El proyecto utiliza las bibliotecas glib, gtk, y gio para la interfaz gráfica. Asegúrate de tener las bibliotecas GTK instaladas en tu sistema. Dependiendo de tu sistema operativo, puedes instalarlas de la siguiente manera:

Linux (Ubuntu/Debian):

       
Ejecuta el siguiente comando:
    1. sudo apt-get install libgtk-3-dev

macOs(con Homebrew):
       
Ejecuta el siguiente comando:

    1. sudo dnf install gtk3-devel

Para Windows: Puedes descargar las bibliotecas GTK desde el sitio oficial:
    • 
    • brew install gtk+3
    • Windows: Puedes descargar las bibliotecas GTK desde el sitio oficial: https://www.gtk.org/download/windows.php.

Configuración del Archivo Cargo.toml:
Antes de ejecutar el proyecto, asegúrate de que el archivo Cargo.toml esté configurado correctamente. Debe contener las dependencias necesarias para GTK y otras especificaciones. Aquí está cómo debe lucir el archivo Cargo.toml:

toml
Copy code
[package]
name = "calculator"
version = "0.1.0"
edition = "2018"

[dependencies]
glib = "0.10.3"
gtk = { version = "0.9.1", features = ["v3_16"] }
gio = { version = "0.9.1", features = ["v2_44"] }

[package.metadata.bundler]
rust-version = "1.51.0"
Asegúrate de guardar este archivo en el directorio raíz de tu proyecto.



Pasos para Ejecutar el Proyecto:
    • Clonar el Repositorio: Primero, clona el repositorio del proyecto en tu máquina local. Puedes hacerlo usando Git o simplemente descargando el código fuente como un archivo ZIP desde GitHub.
      
    • Abrir una Terminal o Línea de Comandos: Abre una terminal o línea de comandos en el directorio donde se encuentra el archivo Cargo.toml.

    • Instalar Dependencias: Antes de compilar el proyecto, asegúrate de que todas las dependencias estén instaladas. Abre una terminal y ejecuta los siguientes comandos:
      
    1. cargo update
    2. Esto actualizará las dependencias del proyecto según las versiones especificadas en el archivo Cargo.toml.

Compilar el Proyecto: Una vez que todas las dependencias estén instaladas, puedes compilar el proyecto ejecutando el siguiente comando:


    1. cargo build
    2. Esto instalará las dependencias necesarias y generará el ejecutable del proyecto en el directorio target/debug/.


Ejecutar el Proyecto: Para ejecutar el proyecto, simplemente ingresa el siguiente comando en la terminal:
    1.      cargo run
    2. Esto iniciará la aplicación de la calculadora con la interfaz gráfica creada con GTK.
       
Usar la Calculadora: Aparecerá una ventana de la calculadora con un campo de entrada de texto, un botón "Calcular" y una etiqueta para mostrar mensajes. Ingresa los números y el operador separados por espacios en el campo de entrada y luego haz clic en el botón "Calcular". La aplicación realizará la operación matemática y mostrará el resultado en la etiqueta.

Personalizar el Estilo (Opcional): Si deseas modificar el estilo de la calculadora, puedes editar el contenido de la variable style_data en el archivo fuente. Esto te permitirá cambiar la apariencia de la interfaz gráfica.






Ejecución de Pruebas (Opcional):
El proyecto incluye pruebas unitarias en la sección [cfg(test)]. Si deseas ejecutar las pruebas para asegurarte de que todo funcione correctamente, puedes utilizar el siguiente comando:

    1. cargo test
    2. Esto ejecutará todas las pruebas definidas en la sección de pruebas y mostrará los resultados.

Con estos pasos y requisitos, deberías poder ejecutar y utilizar la calculadora GTK sin problemas. Si encuentras algún problema o tienes alguna otra pregunta, no dudes en preguntar. ¡Disfruta de tu calculadora interactiva!
