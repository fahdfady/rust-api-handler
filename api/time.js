const handler = () => {
    const now = new Date();

    return {
        status: 200,
        body: JSON.stringify({
            timestamp: now.getTime(),
            iso: now.toISOString(),
            message: "Current server time"
        })
    };

}