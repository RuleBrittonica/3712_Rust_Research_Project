import { RemDaemonClient } from './client';
import { JsonResp, VerifyPayload, VerifyData } from './interface';

export async function runVerify(
  _client: RemDaemonClient,
  _payload: VerifyPayload
): Promise<VerifyData> {
  // TODO: implement when server supports { op: "verify" }
  throw new Error('Verification not implemented yet (server op "verify" pending)');
}
