import Image from "next/image";

import { FeedbackForm, FeedbackTable } from "./interactions";


async function loadFeedbacks() {
  const res = await fetch(`${process.env.ENDPOINT}/zephyr/execute`, {
    method: 'POST',
    headers: {
      Authorization: ['Bearer', process.env.MERCURY_JWT].join(' '),
      'Content-Type': 'application/json'
    },
    cache: "no-cache",
    body: JSON.stringify({
      mode: {
        "Function": {
          fname: "feedbacks",
          arguments: ""
        }
      }
    })
  })

  const json_res = await res.json()
  return json_res
}

export default async function Home() {
  let feedback = await loadFeedbacks();
  feedback = feedback.sort((a,b) => b.votes - a.votes);
  
  return (
    <main className="flex min-h-screen flex-col items-center p-24">
      <div className="my-20">
        <h1 className="text-2xl">On-Chain Stellar Complaints and Feedback</h1>
        <p className="text-md">The first RPC-less app on Soroban. Learn more about the app in the blog post.</p>
      </div>
      <div>
      <div className="relative overflow-x-auto">
        <FeedbackForm></FeedbackForm>
        <FeedbackTable feedback={feedback}></FeedbackTable>
</div>
      </div>
    </main>
  );
}
