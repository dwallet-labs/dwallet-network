import {Keypair} from "../cryptography";
import {SuiClient} from "../client";
import {setTimeout} from "timers/promises";

export async function fetchObjectBySessionId(sessionId: string, type: string,keypair: Keypair, client: SuiClient) {
    let cursor = null;
    while(true) {
        const objects = await client.getOwnedObjects({ owner: keypair.toSuiAddress(), cursor: cursor });
        // @ts-ignore
        const objectsContent = await client.multiGetObjects({ ids: objects.data.map((o) => o.data?.objectId)!, options: { showContent: true } });


        const objectsFiltered = objectsContent.map((o) => o.data?.content).filter((o) => {
            // @ts-ignore
            return o?.dataType == "moveObject" && o?.type == type && o.fields["session_id"] == sessionId;

        });
        if(objectsFiltered.length > 0) {
            return objectsFiltered[0];
        } else if (objects.hasNextPage) {
            cursor = objects.nextCursor;
        } else {
            cursor = null;
        }
        await setTimeout(500);
    }
}