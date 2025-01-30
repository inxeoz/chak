
const router = express.Router();
const resultService = new resultService();

router.post('/result', async (req, res) => {
  const resultDto = req.body;
  try {
    const result = await resultService.addResult(resultDto);
    res.status(201).json(result);
  } catch (error) {
    res.status(500).json({ message: error.message });
  }
});

export default router;