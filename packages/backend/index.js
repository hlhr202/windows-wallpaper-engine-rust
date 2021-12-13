import { fileURLToPath } from 'url';
import { createRequire } from 'module';

const __filename = fileURLToPath(import.meta.url);
const require = createRequire(__filename); 
const { setWallpaper } = require("./index.win32-x64-msvc.node");

export { setWallpaper };
