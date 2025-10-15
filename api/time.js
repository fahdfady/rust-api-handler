const handler = () => {
    const now = new Date();

    return JSON.stringify({
        status: 200,
        body: {
            timestamp: now.getTime(),
            iso: now.toISOString(),
            message: "Current server time"
        }
    });
}