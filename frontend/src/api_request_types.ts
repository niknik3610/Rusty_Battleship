import { Game } from "./game";
import { Request } from "./api_request";

export module ApiRequestType {
    export type MoveRequest = {
        coordinates: [number, number];
        moveType: Game.MoveType;
    };
    export class MoveRequestGroup {
        moveRequests: MoveRequest[];

        public constructor() {
            this.moveRequests = [];
        }

        public push(coordinates: [number, number], moveType: Game.MoveType) {
            let newRequest: MoveRequest = {
                coordinates: coordinates,
                moveType: moveType,
            };
            this.moveRequests.push(newRequest);
        }

        public async resolve(client_id: number) {
            if (this.moveRequests.length < 1) {
                return this.moveRequests;
            }
            let req_content = JSON.stringify(this.moveRequests);
            req_content = `{"moveRequests":${req_content}}`;
            this.moveRequests = [];

            Request.postRequest(`api/sendMove/{${client_id}}`, req_content);
        }
    }
}
