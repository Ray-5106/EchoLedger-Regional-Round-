const express = require('express');
const bodyParser = require('body-parser');
const crypto = require('crypto');
const AWS = require('aws-sdk');

const s3 = new AWS.S3({ region: "us-east-1" });
const kms = new AWS.KMS({ region: "us-east-1" });

const app = express();
app.use(bodyParser.json({ limit: '5mb' }));

app.post('/store_phi', async (req, res) => {
  try {
    const phi = JSON.stringify(req.body.phi);
    const dataKey = crypto.randomBytes(32);
    const iv = crypto.randomBytes(12);
    const cipher = crypto.createCipheriv('aes-256-gcm', dataKey, iv);
    let encrypted = cipher.update(phi, 'utf8', 'base64');
    encrypted += cipher.final('base64');
    const tag = cipher.getAuthTag().toString('base64');

    const kmsResp = await kms.encrypt({
      KeyId: process.env.KMS_KEY_ID,
      Plaintext: dataKey
    }).promise();

    const key = `phi/${crypto.createHash('sha256').update(phi).digest('hex')}-${Date.now()}.enc`;
    await s3.putObject({
      Bucket: process.env.PHI_BUCKET,
      Key: key,
      Body: Buffer.from(encrypted, 'base64'),
      Metadata: { iv: iv.toString('base64'), tag: tag }
    }).promise();

    const offChainRef = JSON.stringify({
      storage: "s3",
      bucket: process.env.PHI_BUCKET,
      key,
      kms_cipher: kmsResp.CiphertextBlob.toString('base64')
    });

    res.json({ off_chain_ref: Buffer.from(offChainRef).toString('base64') });
  } catch (e) {
    res.status(500).json({ error: "failed" });
  }
});

app.listen(3000);

module.exports = app;