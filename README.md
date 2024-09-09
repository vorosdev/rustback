### Documentación 

```markdown
# rustback

`rustback` es un backdoor implementado en Rust que permite a los clientes ejecutar comandos
 de shell de forma remota. El servidor escucha en el puerto 4444 y maneja múltiples conexiones
 concurrentemente utilizando Tokio.

## Características

- Manejo de múltiples conexiones asíncronas.
- Ejecución remota de comandos de shell (`sh`).
- Devuelve la salida de los comandos al cliente.
- El nombre del proceso se establece como `rustback`.

## Requisitos

- Rust 1.56 o superior.

## Instalación

1. Clona este repositorio:
   ```bash
   git clone https://github.com/vorosdev/rustback.git
   ```

2. Entra en el directorio del proyecto:
   ```bash
   cd rustback
   ```

3. Compila el proyecto:
   ```bash
   cargo build --release
   ```

## Uso

1. Ejecuta el servidor:
   ```bash
   ./rustback
   ```

2. El servidor comenzará a escuchar en la dirección `0.0.0.0:4444`.

3. Puedes conectarte al servidor usando cualquier cliente TCP, como `netcat` o `telnet`, y enviar comandos que serán ejecutados en la shell del servidor.

   Ejemplo con `netcat`:
   ```bash
   nc IP 4444
   ```

4. Envía comandos de shell y recibirás la salida directamente en tu terminal.

## Detalles Técnicos

- **Asíncronía**: El servidor utiliza Tokio para gestionar múltiples clientes simultáneamente. Cada conexión se maneja de forma independiente y no bloquea las demás conexiones.
- **Ejecución de comandos**: El servidor ejecuta los comandos recibidos en una shell (`sh`) y devuelve la salida estándar (`stdout`) al cliente.
- **Nombre del proceso**: Utiliza `prctl` para establecer el nombre del proceso a `rustback`, lo que facilita su identificación en herramientas como `ps` o `top`.

## Notas

- El servidor está diseñado para entornos seguros o controlados, ya que permite la ejecución remota de comandos en una shell. No se recomienda su uso en sistemas de producción sin las debidas precauciones de seguridad.
- La salida de errores estándar (`stderr`) no es enviada al cliente en la versión actual.

## Licencia

Este proyecto está bajo la licencia MIT. Consulta el archivo [LICENSE](LICENSE) para más detalles.

## Disclaimer

Descargo de responsabilidad. Consulta el archivo [DISCLAIMER](DISCLAIMER.md) para más detalles.

