import { useState } from 'react'

function App() {
  const [connected, setConnected] = useState(false)

  const connectWallet = async () => {
    try {
      // Freighter wallet integration will go here
      console.log('Connecting to Freighter wallet...')
      setConnected(true)
    } catch (error) {
      console.error('Failed to connect wallet:', error)
    }
  }

  return (
    <div className="min-h-screen bg-gray-50">
      <nav className="bg-white shadow-sm">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between h-16 items-center">
            <div className="flex items-center">
              <h1 className="text-2xl font-bold text-blue-600">Learna2Earna</h1>
            </div>
            <button
              onClick={connectWallet}
              className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition"
            >
              {connected ? 'Connected' : 'Connect Wallet'}
            </button>
          </div>
        </div>
      </nav>

      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        <div className="text-center">
          <h2 className="text-4xl font-bold text-gray-900 mb-4">
            Learn to Earn, On-chain
          </h2>
          <p className="text-xl text-gray-600 mb-8">
            A decentralized learning platform where educators create quests,
            learners complete milestones, and smart contracts handle rewards.
          </p>
          
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mt-12">
            <div className="bg-white p-6 rounded-lg shadow-md">
              <h3 className="text-xl font-semibold mb-2">Create Quests</h3>
              <p className="text-gray-600">
                Set up learning paths with milestones and token rewards
              </p>
            </div>
            
            <div className="bg-white p-6 rounded-lg shadow-md">
              <h3 className="text-xl font-semibold mb-2">Complete Milestones</h3>
              <p className="text-gray-600">
                Prove your progress and earn rewards for each achievement
              </p>
            </div>
            
            <div className="bg-white p-6 rounded-lg shadow-md">
              <h3 className="text-xl font-semibold mb-2">Earn Tokens</h3>
              <p className="text-gray-600">
                Get paid automatically through smart contracts
              </p>
            </div>
          </div>
        </div>
      </main>
    </div>
  )
}

export default App
