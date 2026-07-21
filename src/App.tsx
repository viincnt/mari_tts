import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { AddSoundDialog } from "@/components/AddSoundDialog";
import { SoundTile } from "@/components/SoundTile";
import type { Sound } from "@/types";

function App() {
  const [sounds, setSounds] = useState<Sound[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    invoke<Sound[]>("list_sounds")
      .then(setSounds)
      .finally(() => setLoading(false));
  }, []);

  return (
    <main className="mx-auto min-h-screen max-w-3xl px-6 py-8">
      <header className="mb-8 flex items-center justify-between gap-4">
        <div>
          <h1 className="font-mono text-lg font-bold tracking-widest uppercase">
            Mari <span className="text-primary">TTS</span>
          </h1>
          <p className="text-muted-foreground font-mono text-xs tracking-wide">
            Robot-voice soundboard
          </p>
        </div>
        <AddSoundDialog onCreated={(s) => setSounds((prev) => [...prev, s])} />
      </header>

      {loading ? (
        <p className="text-muted-foreground font-mono text-sm">Loading…</p>
      ) : sounds.length === 0 ? (
        <div className="border-border text-muted-foreground rounded-md border border-dashed p-10 text-center font-mono text-sm">
          No clips yet. Hit "New sound" to record your first one.
        </div>
      ) : (
        <div className="grid grid-cols-2 gap-3 sm:grid-cols-3 md:grid-cols-4">
          {sounds.map((sound) => (
            <SoundTile
              key={sound.id}
              sound={sound}
              onDeleted={(id) =>
                setSounds((prev) => prev.filter((s) => s.id !== id))
              }
            />
          ))}
        </div>
      )}
    </main>
  );
}

export default App;
