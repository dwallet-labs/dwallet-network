// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import React from "react";
export default function FooterCopyright({ copyright }) {
  return (
    <div
      className="text-sm lg:text-xs xl:text-sm mt-2"
      // Developer provided the HTML, so assume it's safe.
      // eslint-disable-next-line react/no-danger
      dangerouslySetInnerHTML={{ __html: copyright }}
    />
  );
}
