const express = require('express');
const app = express();
app.get('/', (req, res) => {
    res.send('Error: 404')
});

app.get('/v1/', (req, res) => {
    res.send('Welcome to the jonas_jones-api!')
});

app.get('/v1/projects', (req, res) => {
    const keyword = req.query.keyword;
    const version = req.query.version;
    const platform = req.query.platform;
    const topic = req.query.topic;
    return res.status(200).json({
        success: true,
        projects: {
            keyword: keyword,
            version: version,
            platform: platform,
            topic: topic,
        }
    })
    

});
app.listen(8000, () => {
    console.log('api listening on port 8000!')
});