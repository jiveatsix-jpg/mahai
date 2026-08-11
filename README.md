# Ma'hai

Centro de lanzamiento (hub) con estética LCARS para el ecosistema de apps de jiveatsix.

Instala y lanza cuatro apps de escritorio desde una sola ventana:

- **NioBiologic**
- **Captain's Log**
- **PixelProcessor**
- **AntiCloud**

## Descargar

El instalador combinado (`MahaiSetup.exe`) que instala Ma'hai + las 4 apps con accesos
directos en Escritorio y Menú Inicio está disponible en
[Releases](../../releases).

## Desarrollo

```bash
npm install
npm run tauri dev
```

## Build

```bash
npm run tauri build
```

El binario resultante (`src-tauri/target/release/mahai.exe`) espera encontrar los
ejecutables de las 4 apps (`NioBiologic.exe`, `CaptainsLog.exe`, `PixelProcessor.exe`,
`AntiCloud.exe`) en su misma carpeta de instalación.
