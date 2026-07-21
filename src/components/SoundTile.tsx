import { useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";
import { Play, Square, Trash2 } from "lucide-react";
import type { Sound } from "@/types";
import { cn } from "@/lib/utils";

export function SoundTile({
  sound,
  onDeleted,
}: {
  sound: Sound;
  onDeleted: (id: string) => void;
}) {
  const [playing, setPlaying] = useState(false);
  const [loading, setLoading] = useState(false);
  const [deleting, setDeleting] = useState(false);
  const audioRef = useRef<HTMLAudioElement | null>(null);
  const urlRef = useRef<string | null>(null);

  function stop() {
    audioRef.current?.pause();
    audioRef.current = null;
    if (urlRef.current) {
      URL.revokeObjectURL(urlRef.current);
      urlRef.current = null;
    }
    setPlaying(false);
  }

  async function play() {
    if (playing) {
      stop();
      return;
    }
    setLoading(true);
    try {
      const bytes = await invoke<number[]>("get_sound_audio", {
        id: sound.id,
      });
      const blob = new Blob([new Uint8Array(bytes)], { type: "audio/wav" });
      const url = URL.createObjectURL(blob);
      urlRef.current = url;

      const audio = new Audio(url);
      audioRef.current = audio;
      audio.onended = stop;
      setPlaying(true);
      await audio.play();
    } catch (err) {
      console.error(err);
      stop();
    } finally {
      setLoading(false);
    }
  }

  async function handleDelete(e: React.MouseEvent) {
    e.stopPropagation();
    stop();
    setDeleting(true);
    try {
      await invoke("delete_sound", { id: sound.id });
      onDeleted(sound.id);
    } catch (err) {
      console.error(err);
      setDeleting(false);
    }
  }

  return (
    <div
      role="button"
      tabIndex={0}
      onClick={play}
      onKeyDown={(e) => {
        if (e.key === "Enter" || e.key === " ") {
          e.preventDefault();
          play();
        }
      }}
      aria-disabled={loading || deleting}
      className={cn(
        "group border-border bg-card relative flex aspect-square cursor-pointer flex-col items-center justify-center gap-2 rounded-md border p-3 text-center shadow-sm transition-colors",
        "hover:border-primary/60 hover:bg-secondary/60",
        playing &&
          "border-primary bg-primary/10 shadow-[0_0_0_1px_var(--primary)]",
        (loading || deleting) && "pointer-events-none opacity-60",
      )}
    >
      <span
        className={cn(
          "absolute top-2 left-2 size-2 rounded-full bg-red-500/30",
          playing && "animate-pulse bg-red-500",
        )}
      />

      {playing ? (
        <Square className="text-primary size-6 fill-current" />
      ) : (
        <Play className="text-muted-foreground group-hover:text-foreground size-6" />
      )}

      <span className="line-clamp-2 max-w-full font-mono text-xs tracking-wide uppercase">
        {sound.label}
      </span>

      <span className="text-muted-foreground absolute right-2 bottom-1.5 font-mono text-[10px] uppercase opacity-70">
        {sound.lang}
      </span>

      <Button
        variant="ghost"
        size="icon"
        onClick={handleDelete}
        disabled={deleting}
        className="text-muted-foreground hover:text-destructive absolute top-1 right-1 size-6 opacity-0 group-hover:opacity-100"
      >
        <Trash2 className="size-3.5" />
      </Button>
    </div>
  );
}
