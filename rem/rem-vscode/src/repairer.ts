import { RemDaemonClient } from './client';
import { JsonResp, RepairPayload, RepairData } from './interface';

export async function runRepair(
  _client: RemDaemonClient,
  _payload: RepairPayload
): Promise<RepairData> {
  // TODO: implement when server supports { op: "repair" }
  throw new Error('Repair not implemented yet (server op "repair" pending)');
}
