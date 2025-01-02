
import React, { useState } from 'react';

const MintForm = () => {
  const [title, setTitle] = useState('');
  const [artist, setArtist] = useState('');
  const [file, setFile] = useState<File | null>(null);

  const handleMint = async (e: React.FormEvent) => {
    e.preventDefault();
    // Add minting logic here
  };

  return (
    <form onSubmit={handleMint}>
      <input
        type="text"
        placeholder="Title"
        value={title}
        onChange={(e) => setTitle(e.target.value)}
      />
      <input
        type="text"
        placeholder="Artist"
        value={artist}
        onChange={(e) => setArtist(e.target.value)}
      />
      <input
        type="file"
        onChange={(e) => setFile(e.target.files?.[0] || null)}
      />
      <button type="submit">Mint NFT</button>
    </form>
  );
};

export default MintForm;
