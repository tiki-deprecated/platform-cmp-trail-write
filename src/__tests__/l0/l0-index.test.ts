/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

import * as l0Index from "../../l0/index/l0-index";
import * as base64 from "../../utils/base64";

describe("L0-Index Tests", function () {
  test("TxnList Success", async () => {
    const content =
      "/QABL/b00Rh3Pn9ADTlqLTOlSqgtJBSUk27Ath8VepihUYAKUAJjL/qwfK6MsGPInGBYvXnbkqD/5Es9PNqnDZeJtHsNsUA3bhzsqMH6X8XfPWU71ZGbrkwSilM3094x1e6gVDbL7uTAHO3THTiEq5uY0qCDyd+G8tky3N3UwQq3lgc56iUNXda84y+MDVlxspHPvO1CBFPJjsfM7gU5VwD9oUGtZhxR6ZtBO21JBhZEOIHSzN+3rxgOmVqYFZQPRoRdXiNoRjMjZgG3fep5cIgrK6eSX5Z2oOdyb033PiLmBKjLio83x6noJAu69kVDl88KcP5JijdB/E29KSBkK3uoVP2wAQEBBGPO5gEgqbplkmHccgNwpcXqZmAOFAE5CLYbV6IeKgGLA+NVp78gpTG3DsZR2xBXr76Ck3XxDQS7c+x7rKC4+EgoZDlNr4kBAf1iAQEBINCVucYQDPyNJMgpDq6RwaSfE/vcylPs4kXTYYYvXigmBGPO5dwBAP0AAUtJLa6HTj4QjYDkCxkLhPMtBW8Zp5dUchYIlL7Hekb6j/nghCuW7+0at7NALhHR8CzORA430BSN4SLcr6AWENS2Ap4Oy8TkDuBnk4F5mToM5CWp/pnGxJNvEhhmCGFSUa4oYMH3+Ho8rWvdWioGEz1I6UDzVl0qhQO+1rWr75zKyKP6hgiq12XUI6rylHRQwjWmNMRSaXN+gEoURVwKzpgDm5Wxo4U8/p+YNbRs6SkJ96xovzuqrAieWRnDns05XxGIiz6j3R/s51MRmEqNIiIcoFoXBR9CG64YwXz75DXsa87ttSatqJW78o+9T7pkNmTnEKy5hPi5+YCwRLfZtZw0INcbgh8plh39dHSTqiW+0QE8Y10JrOFFCnvRAuSFB1gJDAVbIioiXQVbIioiXQEAAQABAA==";
    const contentBytes = base64.decode(content);
    const txnList = await l0Index.txnList(contentBytes);

    expect(txnList.length).toBe(1);
    expect(txnList[0]).toBe("aUoaabtrKe8bA2oQL3f_PwhWG2rFJprZ-D3PHc2O-Gk");
  });
});
