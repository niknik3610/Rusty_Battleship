export module Request {
    export async function getRequest(url: string): Promise<string> {
        let response = await fetch(url, {
            method: "GET",
        });

        return response.text();
    }

    export async function postRequest(
        url: string,
        contents: string,
    ): Promise<string> {
        let response = await fetch(url, {
            method: "POST",
            body: contents,
            headers: {
                "Content-type": "application/json; charset=UTF-8",
            },
        });

        return response.text();
    }
}
