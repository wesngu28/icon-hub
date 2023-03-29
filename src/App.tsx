import { useEffect } from 'react'
import { invoke } from "@tauri-apps/api";

function App() {

  useEffect(() => {
    async function test() {
      const names = await invoke('tester')
      console.log(names)
    }
    test()
  }, [])

  return (
    <div className='p-4 flex gap-4'>
      <div className='rounded-md w-32 bg-gray-800 text-white'>
        <h1 className='p-1 text-center'>Taskbar</h1>
      </div>
      <div className='rounded-md w-32 bg-gray-800 text-white'>
        <h1 className='p-1 text-center'>Desktop Icons</h1>
      </div>
      <div className='flex-1'>
        <h1>Content</h1>
      </div>
    </div>
  )
}

export default App
