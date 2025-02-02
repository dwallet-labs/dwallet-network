import { Transaction } from "@ika-io/ika/transactions";
import { Button, Container } from "@radix-ui/themes";
import { useSignAndExecuteTransaction, useIkaClient } from "@mysten/dapp-kit";
import { useNetworkVariable } from "./networkConfig";
import ClipLoader from "react-spinners/ClipLoader";

export function CreateCounter({
  onCreated,
}: {
  onCreated: (id: string) => void;
}) {
  const counterPackageId = useNetworkVariable("counterPackageId");
  const ikaClient = useIkaClient();
  const {
    mutate: signAndExecute,
    isSuccess,
    isPending,
  } = useSignAndExecuteTransaction();

  function create() {
    const tx = new Transaction();

    tx.moveCall({
      arguments: [],
      target: `${counterPackageId}::counter::create`,
    });

    signAndExecute(
      {
        transaction: tx,
      },
      {
        onSuccess: async ({ digest }) => {
          const { effects } = await ikaClient.waitForTransaction({
            digest: digest,
            options: {
              showEffects: true,
            },
          });

          onCreated(effects?.created?.[0]?.reference?.objectId!);
        },
      },
    );
  }

  return (
    <Container>
      <Button
        size="3"
        onClick={() => {
          create();
        }}
        disabled={isSuccess || isPending}
      >
        {isSuccess || isPending ? <ClipLoader size={20} /> : "Create Counter"}
      </Button>
    </Container>
  );
}
