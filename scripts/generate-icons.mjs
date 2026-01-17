import sharp from 'sharp';
import { readFileSync, mkdirSync, existsSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const iconsDir = join(__dirname, '..', 'src-tauri', 'icons');
const svgPath = join(iconsDir, 'app-icon.svg');

// Read the SVG file
const svgBuffer = readFileSync(svgPath);

// Icon sizes needed for Tauri
const sizes = [
  { name: '32x32.png', size: 32 },
  { name: '128x128.png', size: 128 },
  { name: '128x128@2x.png', size: 256 },
  { name: 'icon.png', size: 512 },
  // Windows Store logos
  { name: 'Square30x30Logo.png', size: 30 },
  { name: 'Square44x44Logo.png', size: 44 },
  { name: 'Square71x71Logo.png', size: 71 },
  { name: 'Square89x89Logo.png', size: 89 },
  { name: 'Square107x107Logo.png', size: 107 },
  { name: 'Square142x142Logo.png', size: 142 },
  { name: 'Square150x150Logo.png', size: 150 },
  { name: 'Square284x284Logo.png', size: 284 },
  { name: 'Square310x310Logo.png', size: 310 },
  { name: 'StoreLogo.png', size: 50 },
];

async function generateIcons() {
  console.log('Generating icons from SVG...');

  for (const { name, size } of sizes) {
    const outputPath = join(iconsDir, name);
    await sharp(svgBuffer)
      .resize(size, size)
      .png()
      .toFile(outputPath);
    console.log(`  Created ${name} (${size}x${size})`);
  }

  // Generate ICO for Windows (using the 256x256 as base)
  console.log('  Note: For icon.ico, please use an online converter or tool like png2ico');
  console.log('        The 256x256 PNG can be converted at: https://icoconvert.com/');

  // Generate ICNS for macOS (using the 512x512 as base)
  console.log('  Note: For icon.icns, use: iconutil -c icns iconset on macOS');
  console.log('        Or use an online converter: https://cloudconvert.com/png-to-icns');

  console.log('\nDone! PNG icons have been generated.');
}

generateIcons().catch(console.error);
