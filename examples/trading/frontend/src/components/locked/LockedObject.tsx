// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { CONSTANTS } from "@/constants";
import { useIkaClientQuery } from "@mysten/dapp-kit";
import { Locked } from "./partials/Locked";
import { IkaObjectData } from "@ika-io/ika/client";

/**
 * Acts as a wrapper between the `Locked` object fetched from API
 * and the on-chain object state.
 *
 * Accepts an `object` of type `::locked::Locked`, fetches the itemID (though the DOF)
 * and then renders the `Locked` component.
 *
 * ItemId is optional because we trust the API to return the correct itemId for each Locked.
 */
export function LockedObject({
  object,
  itemId,
  hideControls,
}: {
  object: IkaObjectData;
  itemId?: string;
  hideControls?: boolean;
}) {
  const owner = () => {
    if (
      !object.owner ||
      typeof object.owner === "string" ||
      !("AddressOwner" in object.owner)
    )
      return undefined;
    return object.owner.AddressOwner;
  };

  const getKeyId = (item: IkaObjectData) => {
    if (
      !(item.content?.dataType === "moveObject") ||
      !("key" in item.content.fields)
    )
      return "";
    return item.content.fields.key as string;
  };

  // Get the itemID for the locked object (We've saved it as a DOF on the SC).
  const ikaObjectId = useIkaClientQuery(
    "getDynamicFieldObject",
    {
      parentId: object.objectId,
      name: {
        type: CONSTANTS.escrowContract.lockedObjectDFKey,
        value: {
          dummy_field: false,
        },
      },
    },
    {
      select: (data) => data.data,
      enabled: !itemId,
    },
  );

  return (
    <Locked
      locked={{
        itemId: itemId || ikaObjectId.data?.objectId!,
        objectId: object.objectId,
        keyId: getKeyId(object),
        creator: owner(),
        deleted: false,
      }}
      hideControls={hideControls}
    />
  );
}
