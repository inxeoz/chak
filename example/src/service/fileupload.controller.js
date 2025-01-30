import express from 'express';
import { createReadStream } from 'fs';
import archiver from 'archiver';
import multer from 'multer';

const upload = multer({ dest: 'uploads/' });
const fileRouter = express.Router();

let gridFsBucket; // Assume this is injected externally

// Middleware to inject gridFsBucket
fileRouter.use((req, res, next) => {
  if (!gridFsBucket) {
    return res.status(500).json({ message: "GridFS bucket not initialized" });
  }
  next();
});

// Upload file
fileRouter.post('/upload', upload.single('file'), async (req, res) => {
  const file = req.file;
  const { mimetype, path, filename, fieldname } = file;

  const stream = createReadStream(path).pipe(
    gridFsBucket.openUploadStream(filename, {
      contentType: mimetype,
      metadata: { fieldname },
    })
  );

  stream.on('finish', () => res.status(200).send('File uploaded successfully'));
  stream.on('error', (err) => res.status(500).send(err.message));
});

// List all files
fileRouter.get('/files', async (req, res) => {
  const files = await gridFsBucket.find({}).toArray();
  res.status(200).json(files);
});

// Download single file
fileRouter.get('/file/:filename', async (req, res) => {
  const { filename } = req.params;
  const files = await gridFsBucket.find({ filename }).toArray();
  const file = files.length > 0 ? files[0] : null;

  if (!file) {
    return res.status(404).send('File not found');
  }

  const downloadStream = gridFsBucket.openDownloadStream(file._id);
  downloadStream.pipe(res);
});

// Download multiple files as ZIP
fileRouter.get('/download', async (req, res) => {
  const { filenames } = req.query;

  if (!filenames || filenames.length === 0) {
    return res.status(400).json({ message: 'No files provided' });
  }

  const archive = archiver('zip', { zlib: { level: 9 } });
  res.attachment('files.zip');
  archive.pipe(res);

  for (const filename of filenames) {
    const files = await gridFsBucket.find({ filename }).toArray();
    const file = files.length > 0 ? files[0] : null;

    if (!file) {
      return res.status(404).json({ message: `File ${filename} not found` });
    }

    const downloadStream = gridFsBucket.openDownloadStream(file._id);
    downloadStream.on('error', (err) => {
      console.error(`Error downloading file ${filename}:`, err);
      return res.status(500).json({ message: `Error downloading file ${filename}` });
    });

    archive.append(downloadStream, { name: filename });
  }

  archive.finalize();
});

export default fileRouter;

// Function to inject gridFsBucket externally
export const setGridFsBucket = (bucket) => {
  gridFsBucket = bucket;
};
