export async function getRequest(url: string): Promise<string> {
    let response = await fetch(url, {
        method: 'get',   
    });

    return response.text()
}
