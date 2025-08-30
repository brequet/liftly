// scripts/generate-image-manifest.mjs
import fs from 'fs/promises';
import path from 'path';

const exercisesDir = path.resolve('static/exercises');
const outputDir = path.resolve('src/lib/generated');
const outputPath = path.join(outputDir, 'exercise-images.json');

async function generateManifest() {
	try {
		console.log('Scanning for exercise images...');
		const files = await fs.readdir(exercisesDir);
		const imageMap = {};

		for (const file of files) {
			if (file.endsWith('.svg')) {
				const [id, type] = file.replace('.svg', '').split('-');
				if (!id || !type) continue;

				if (!imageMap[id]) {
					imageMap[id] = [];
				}
				// Store the full path for direct use in `src` attributes
				imageMap[id].push(`/exercises/${file}`);
			}
		}

		// Ensure the output directory exists
		await fs.mkdir(outputDir, { recursive: true });
		// Write the JSON file
		await fs.writeFile(outputPath, JSON.stringify(imageMap, null, 2));

		console.log(`✅ Image manifest generated successfully at ${outputPath}`);
		console.log(`Found images for ${Object.keys(imageMap).length} unique exercises.`);
	} catch (error) {
		if (error.code === 'ENOENT') {
			console.warn(
				`⚠️ static/exercises directory not found. Skipping manifest generation.`
			);
			// Create an empty manifest to prevent build errors
			await fs.mkdir(outputDir, { recursive: true });
			await fs.writeFile(outputPath, JSON.stringify({}));
		} else {
			console.error('❌ Failed to generate image manifest:', error);
			process.exit(1);
		}
	}
}

generateManifest();
